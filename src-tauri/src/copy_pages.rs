use lopdf::Document;

use lopdf::{Object, ObjectId};
use std::collections::BTreeMap;

pub fn main(mut doc: Document) -> Result<Document, &'static str> {
    let page_count = doc.get_pages().len();
    let delete_page_numbers: Vec<u32> = (4..=page_count).map(|x| x as u32).collect();

    if delete_page_numbers.len() <= page_count {
        doc = duplicate(doc).unwrap();
    } else {
        doc.delete_pages(&delete_page_numbers);
    }

    Ok(doc)
}

pub fn duplicate(mut doc: Document) -> Result<Document, &'static str> {
    let copy_page_numbers: Vec<u32> = (1..=3).map(|x| x as u32).collect();

    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    doc.renumber_objects_with(1);

    let pages = doc.get_pages();

    let mut insert_page_number: u32 = 1;
    // This is actually better than extend as we use less allocations and cloning then.
    for (_, object_id) in pages.into_iter() {
        let value = doc.get_object(object_id).unwrap().to_owned();
        if copy_page_numbers.contains(&insert_page_number) {
            documents_pages.insert(object_id, value);
        }
        insert_page_number += 1;
    }

    documents_objects.extend(doc.objects);

    // Catalog and Pages are mandatory
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.into_iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects
        // All other objects should be collected and inserted into the main Document
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages"
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        object_id
                    },
                    object,
                ));
            }
            "Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" => {}     // Ignored, processed later and separately
            "Outlines" => {} // Ignored, not supported yet
            "Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(object_id, object);
            }
        }
    }

    // If no "Pages" found abort
    if pages_object.is_none() {
        return Err("Pages root not found.");
    }

    // Iter over all "Page" and collect with the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found abort
    if catalog_object.is_none() {
        return Err("Catalog root not found.");
    }

    let (catalog_id, catalog_object) = catalog_object.unwrap();
    let (page_id, page_object) = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = page_object.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                .into_iter()
                .map(|(object_id, _)| Object::Reference(object_id))
                .collect::<Vec<_>>(),
        );

        document
            .objects
            .insert(page_id, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", page_id);
        dictionary.set("PageMode", "UseOutlines");
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
            .objects
            .insert(catalog_id, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_id);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    //Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    //Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(ref mut dict)) = document.get_object_mut(catalog_id) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    // Most of the time this does nothing unless there are a lot of streams
    // Can be disabled to speed up the process.
    // document.compress();

    // Save the merged PDF
    // Store file in current working directory.
    // document.save("merged.pdf").unwrap();
    Ok(document)
}
