<script setup lang="ts">
import { Ref, ref } from 'vue'
import DragArea from './components/DragArea.vue'
import EditSettings from './components/EditSettings.vue'
import FileList from './components/FileList.vue'
import { isFileInfoResponse, FileInfo, FileStatus } from './types/fileList'
import { invoke } from '@tauri-apps/api'
import StartBtn from './components/StartBtn.vue'
import { ProcessStatus } from './types/processStatus';

export interface GlobalSettings {
  rangeBegin: number
  rangeEnd: number
  outDir: string
}

const fileList: Ref<FileInfo[]> = ref([])

const updateGlobalSettings = (settings: Partial<GlobalSettings>) => {
  globalSettings.value = {
    ...globalSettings.value,
    ...settings,
  }
}

const setGlobalRange = (begin: number, end: number) => {
  globalSettings.value.rangeBegin = begin
  globalSettings.value.rangeEnd = end
}

const loading = ref(false)
const showFileList = ref(false)

const globalSettings = ref({
  rangeBegin: 0,
  rangeEnd: 0,
  outDir: '',
})

const onSelectedFiles = async (pathList: string[]) => {
  loading.value = true
  showFileList.value = true
  const files = await invoke('load_pdf_files', { pathList: pathList })
  loading.value = false
  let parsed
  if (typeof files === 'string' && Array.isArray((parsed = JSON.parse(files)))) {
    const pageNumbers = fileList.value.map(file => file.pageNum)
    const minPageNumber = Math.min(3, ...pageNumbers)
    setGlobalRange(1, minPageNumber)

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
          status: FileStatus.LOADED,
          pageRange: null,
        })
      }
    })

    updateGlobalSettings({ outDir: fileList.value[0].dir + '/samples' })
    processStatus.value = ProcessStatus.FILE_LOADED
    progress.value = 0
  }
}

const processStatus = ref<(typeof ProcessStatus)[keyof typeof ProcessStatus]>(ProcessStatus.NONE)
const progress = ref<number>(0)

const startGenerating = async () => {
  processStatus.value = ProcessStatus.GENERATING
  for (const file of fileList.value) {
    const params = {
      fileDir: file.dir,
      fileName: file.fileName,
      outDir: globalSettings.value.outDir,
      rangeBegin: file.pageRange?.[0] ?? globalSettings.value.rangeBegin,
      rangeEnd: file.pageRange?.[1] ?? globalSettings.value.rangeEnd,
    }
    const response = await invoke('generate_sample_pdf', params)
    await new Promise<void>(resolve => {
      setTimeout(() => {
        resolve()
      }, 500)}
    )

    progress.value += 1
    if (typeof response === 'string') {
      const parsed = JSON.parse(response)

      if (typeof parsed === 'object' && 'error' in parsed) {
        console.log('parsed', parsed)
        if (parsed.error === false) {
          file.status = FileStatus.SAVED
        } else {
          file.status = FileStatus.SAVE_FAILED
        }
      }
    }
  }
  processStatus.value = ProcessStatus.DONE
}

const removeFile = (index: number) => {
  fileList.value = fileList.value.filter((_, i) => i !== index)
}

const removeAll = () => {
  fileList.value = []
}

const back = () => {
  removeAll()
  showFileList.value = false
  processStatus.value = ProcessStatus.NONE
}
</script>

<template>
  <div class="h-screen w-screen overflow-y-scroll bg-neutral-100 p-6">
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
          :global-settings="globalSettings"
          @update-global-settings="updateGlobalSettings"
        />

        <StartBtn
          :status="processStatus"
          :percentage="progress / (fileList.length || 1) * 100"
          @start-generating="startGenerating"
        />
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
