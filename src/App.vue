<script setup lang="ts">
import { Ref, ref } from 'vue'
import DragArea from './components/DragArea.vue'
import EditSettings from './components/EditSettings.vue'
import FileList from './components/FileList.vue'
import { isFileInfoResponse, FileInfo } from './types/fileList'
import { invoke } from '@tauri-apps/api'

const fileList: Ref<FileInfo[]> = ref([
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkaigi-2024-pamphlet.pdfphperkaigi-2024-pamphlet.pdfphperkaigi-2024-pamphlet.pdfphperkaigi-2024-pamphlet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkaigi-2024-pamaphlet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkaigi-2024-pamphldet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkaigi-s2024-pamphlet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkdaigi-2024-pamphlet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phpaerkaigi-2024-pamphlet.pdf',
  },
  {
    exists: true,
    dir: '/Users/natsuki/Downloads/',
    pageNum: 2,
    fileName: 'phperkdaigi-2024-pamphlet.pdf',
  },
])

const defaultOutDir = ref<string>('')

const setDefaultOutDir = (value: string) => {
  defaultOutDir.value = value
}

const loading = ref(false)
const showFileList = ref(true)

const onSelectedFiles = async (pathList: string[]) => {
  loading.value = true
  showFileList.value = true
  const files = await invoke('load_pdf_files', { pathList: pathList })
  loading.value = false
  let parsed
  if (typeof files === 'string' && Array.isArray((parsed = JSON.parse(files)))) {
    fileList.value.slice(0)
    parsed.forEach(file => {
      if (isFileInfoResponse(file)) {
        const dirArr = file.path.split('/')
        const fileName = dirArr[dirArr.length - 1]
        const dir = dirArr.slice(0, dirArr.length - 1).join('/')

        fileList.value.push({
          exists: file.exists,
          dir: dir,
          pageNum: file.page_num,
          fileName: fileName,
        })
      }
    })

    setDefaultOutDir(fileList.value[0].dir + '/samples')
  }
}

const startGenerating = async () => {
  for (const file of fileList.value) {
    const params = { fileDir: file.dir, fileName: file.fileName, outDir: '/Users/natsuki/Downloads/samples/file.pdf' }
    const response = await invoke('generate_sample_pdf', params)
    console.log('response', response)
  }
}

const removeFile = (index: number) => {
  fileList.value = fileList.value.filter((_, i) => i !== index)
}
</script>

<template>
  <div class="min-w-screen min-h-screen bg-neutral-100 p-6">
    <DragArea
      v-if="fileList.length === 0"
      @select="onSelectedFiles"
    />

    <div v-if="showFileList">
      <div class="flex items-start">
        <EditSettings
          :out-dir="defaultOutDir"
          @update-out-dir="setDefaultOutDir"
        />

        <button
          class="ms-auto rounded-xl bg-violet-500 p-5 text-3xl text-violet-50 shadow-md"
          @click="startGenerating"
        >
          複製を開始
        </button>
      </div>

      <span class="m-8 block h-0 border-[1px] border-solid border-violet-200"></span>

      <FileList
        :loading="loading"
        :file-list="fileList"
        @click-start-btn="startGenerating"
        @remove-file="removeFile"
      ></FileList>
    </div>
  </div>
</template>

<style scoped></style>
