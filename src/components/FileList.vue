<script setup lang="ts">
import { type PropType } from 'vue'
import { FileInfo } from '../types/fileList'
import { setup } from './fileList'
import { EmitType } from './fileList'
import FileCard from './FileCard.vue'

const emits = defineEmits<EmitType>()

const { onClickStartBtn, removeFile } = setup(emits)

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
    <FileCard
      v-for="(file, i) of fileList"
      :key="file.dir"
      :file="file"
      @click-remove-btn="removeFile(i)"
    ></FileCard>

    <div class="fixed bottom-0">
      <button @click="onClickStartBtn">PDF作成</button>
    </div>
  </div>
</template>

<style scoped></style>
