<script setup lang="ts">
import { Ref, ref } from 'vue';
import DragArea from './pages/DragArea.vue'
import EditSettings from './pages/EditSettings.vue'
import { FileInfo, isFileInfo } from './types/fileList'
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
        if(isFileInfo(file)) {
          fileList.value.push(file)
        }
      })
    }
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
    />
  </div>
</template>

<style scoped></style>
