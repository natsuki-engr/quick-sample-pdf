import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";

export default function () {
  const openDialog = async () => {
    let value = await open({
      filters: [{ name: "PDF", extensions: ["pdf"] }],
    })
    if (value === null) {
      return;
    } else if (!Array.isArray(value)) {
      value = [value];
    }

    console.log('value', value)
    // [Log] value – ["/Users/natsuki/Downloads/1121SKY595.pdf"] (1) (dragArea.ts, line 13)
    const files = await invoke('load_pdf_files', { filePath: value[0] })
    console.log('files', files)
    return files
  };

  const debug = async () => {
    const value = [
      "/Users/natsuki/Downloads/1121SKY595.pdf",
      "/Users/natsuki/Downloads/dddd.pdf",
      "/Users/natsuki/Downloads/phperkaigi-2024-pamphlet.pdf"
    ]
    const responseJson = await invoke('load_pdf_files', { filePath: value })
    if(typeof responseJson === 'string')
    console.log('files', JSON.parse(responseJson))
  }

  return {
    openDialog,
    debug,
  }
}
