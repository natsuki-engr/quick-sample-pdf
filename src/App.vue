<script setup lang="ts">
import { Ref, ref } from 'vue'
import DragArea from './components/DragArea.vue'
import EditSettings from './components/EditSettings.vue'
import FileList from './components/FileList.vue'
import { isFileInfoResponse, FileInfo } from './types/fileList'
import { invoke } from '@tauri-apps/api'

const fileList: Ref<FileInfo[]> = ref([])

const defaultRangeEnd = ref<number>(1)
const defaultOutDir = ref<string>('')

const setDefaultOutDir = (value: string) => {
  defaultOutDir.value = value
}

const setDefaultRangeEnd = (value: number) => {
  defaultRangeEnd.value = value
}

const loading = ref(false)
const showFileList = ref(false)

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
          saved: false
        })
      }
    })

    setDefaultOutDir(fileList.value[0].dir + '/samples')

    const pageNumbers = fileList.value.map(file => file.pageNum)
    setDefaultRangeEnd(Math.min(3, ...pageNumbers))
  }
}

const startGenerating = async () => {
  for (const file of fileList.value) {
    const params = { fileDir: file.dir, fileName: file.fileName, outDir: defaultOutDir.value }
    const response = await invoke('generate_sample_pdf', params)

    if (typeof response === 'string') {
      const parsed = JSON.parse(response)

      if(typeof parsed === 'object' && "error" in parsed) {
        if(parsed.error === false) {
          file.saved = true
        }
      }
    }
  }
}

const removeFile = (index: number) => {
  fileList.value = fileList.value.filter((_, i) => i !== index)
}

const removeAll = () => {
  fileList.value = []
}

const back = () => {
  removeAll()
}
</script>

<template>
  <div class="h-screen w-screen bg-neutral-100 p-6">
    <DragArea
      v-if="fileList.length === 0"
      @select="onSelectedFiles"
    />

    <div v-if="showFileList">
      <div class="mb-2">
        <a
          class="relative flex cursor-pointer items-center"
          @click="back"
        >
          <i class="i-bx-chevron-left text-2xl"></i><span class="leading-none">戻る</span></a
        >
      </div>
      <div class="flex items-start">
        <EditSettings
          :default-range-end="defaultRangeEnd"
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
        @remove-file="removeFile"
      ></FileList>
    </div>
  </div>
</template>

<style scoped></style>
