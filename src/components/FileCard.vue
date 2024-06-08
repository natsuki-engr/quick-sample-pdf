<script setup lang="ts">
import { PropType } from 'vue'
import { setup, type EmitType } from './fileCard'

const emits = defineEmits<EmitType>()

interface FileIfo {
  fileName: string
  dir: string
  pageNum: number
}

defineProps({
  file: {
    type: Object as PropType<FileIfo>,
    required: true,
  },
})

const { clickRemoveBtn } = setup(emits)
</script>

<template>
  <div class="group relative mb-5 max-w-5xl cursor-default rounded-xl border-2 border-solid border-violet-500 bg-white p-4">
    <button
      class="invisible absolute right-0 top-0 h-9 w-9 rounded-bl-md rounded-tr-lg bg-violet-500 group-hover:visible"
      @click="clickRemoveBtn"
    >
      <i class="i-bx-x absolute inset-0 m-auto bg-white text-3xl"></i>
    </button>
    <span
      class="block max-w-2xl overflow-hidden text-ellipsis whitespace-nowrap text-xl"
      v-text="file.fileName"
    ></span>
    <div class="flex items-center">
      <span
        class="text-violet-400"
        v-text="file.dir"
      ></span>
      <span
        class="ms-auto"
        v-text="file.pageNum + 'ページ'"
      ></span>
    </div>
  </div>
</template>

<style scoped></style>
