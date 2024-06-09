<script setup lang="ts">
import { PropType } from 'vue'
import { setup, type EmitType } from './fileCard'
import { FileInfo } from '../types/fileList';

const emits = defineEmits<EmitType>()

defineProps({
  file: {
    type: Object as PropType<FileInfo>,
    required: true,
  },
})

const { clickRemoveBtn } = setup(emits)
</script>

<template>
  <div class="group relative mb-5 flex max-w-5xl cursor-default flex-col rounded-xl border-2 border-solid border-violet-500 bg-white px-3 py-4">
    <button
      class="invisible absolute right-0 top-0 h-9 w-9 rounded-bl-md rounded-tr-lg bg-violet-500 group-hover:visible"
      @click="clickRemoveBtn"
    >
      <i class="i-bx-x absolute inset-0 m-auto bg-white text-3xl"></i>
    </button>

    <div class="flex items-center">
      <i v-if="file.saved" class="i-bxs-check-circle text-3xl text-green-500"></i>
      <i v-else class="i-bxs-file-pdf text-3xl text-violet-500"></i>
      <span
        class="max-w-2xl overflow-hidden text-ellipsis whitespace-nowrap text-xl"
        v-text="file.fileName"
      ></span>
    </div>

    <div class="flex items-center">
      <span
        class="ps-2 text-violet-400"
        v-text="file.dir"
      ></span>
      <span
        class="ms-auto"
        v-text="file.pageNum + 'ページ'"
      ></span>
    </div>
  </div>
</template>
