<script setup lang="ts">
import { type PropType } from 'vue'
import { FileInfo } from '../types/fileList'

defineProps({
  loading: {
    type: Boolean,
    required: true,
  },
  fileList: {
    type: Array as PropType<Array<FileInfo>>,
    required: false,
    default: () => [],
  },
})
</script>

<template>
  <div>
    EditSettings
    <div v-if="loading">loading</div>
    <div
      v-for="file of fileList"
      :key="file.path"
    >
      <span v-text="file.path"></span>
      <span v-text="file.page_num + 'ページ'"></span>
    </div>
  </div>
</template>

<style scoped></style>
