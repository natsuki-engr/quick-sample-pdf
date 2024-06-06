<script setup lang="ts">
import { type PropType } from 'vue'
import { FileInfo } from '../types/fileList'
import { setup } from './fileList';
import { EmitType } from './fileList';

const emits = defineEmits<EmitType>()

const {
  onClickStartBtn,
} = setup(emits)

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
    <div
      v-for="file of fileList"
      :key="file.dir"
    >
    <div>
      <span v-text="file.fileName"></span>
      <span v-text="file.pageNum + 'ページ'"></span>
    </div>
      <span v-text="file.dir"></span>
    </div>

    <div class="fixed bottom-0">
      <button @click="onClickStartBtn">PDF作成</button>
    </div>
  </div>
</template>

<style scoped></style>
