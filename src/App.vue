<script setup lang="ts">
import { Ref, ref } from 'vue';
import DragArea from './components/DragArea.vue'
import EditSettings from './components/EditSettings.vue'
import { isFileInfoResponse, FileInfo } from './types/fileList'
import { invoke } from "@tauri-apps/api";

const fileList: Ref<FileInfo[]> = ref([])

const loading = ref(false)
const showFileList = ref(false)

const onSelectedFiles = async (pathList: string[]) => {
  loading.value = true
  showFileList.value = true
  const files = await invoke('load_pdf_files', { pathList: pathList })
  loading.value = false
  if(typeof files === 'string') {
    const parsed = JSON.parse(files)
    fileList.value.slice(0)
    if(Array.isArray(parsed)) {
      parsed.forEach(file => {
        if(isFileInfoResponse(file)) {
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
    }
  }
}

const startGenerating = async () => {
  for(const file of fileList.value) {
    const params = { fileDir: file.dir, fileName: file.fileName, outDir: '/Users/natsuki/Downloads/samples/file.pdf' }
    const response = await invoke('generate_sample_pdf', params)
    console.log('response', response)
  }
}
</script>

<template>
  <div class="">
    <DragArea @select="onSelectedFiles" />
    <EditSettings
      v-if="showFileList"
      :loading="loading"
      :file-list="fileList"
      @click-start-btn="startGenerating"
    />
  </div>
</template>

<style scoped></style>
