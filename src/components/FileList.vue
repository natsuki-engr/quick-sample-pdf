<script setup lang="ts">
import { type PropType } from 'vue'
import { FileInfo } from '../types/fileList'
import { setup } from './editSettings';
import { EmitType } from './editSettings';

const emits = defineEmits<EmitType>()

const {
  rangeBegin,
  rangeEnd,
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
    <div>
      <span>ファイル分割</span>
      <input v-model="rangeBegin" type="number">
      <span>~</span>
      <input v-model="rangeEnd" type="number">
    </div>
    <div v-if="loading">loading</div>
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
