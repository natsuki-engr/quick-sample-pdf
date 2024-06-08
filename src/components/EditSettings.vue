<script setup lang="ts">
import { PropType } from 'vue'
import { EmitType, setup } from './editSettings'

const emits = defineEmits<EmitType>()

defineProps({
  outDir: {
    type: String as PropType<string>,
    required: true,
  },
})

const { rangeBegin, rangeEnd, openDialog } = setup(emits)
</script>

<template>
  <div class="inline-block min-w-[50%] rounded-lg border-violet-500 bg-white p-6 shadow-md">
    <div class="mb-4 flex items-center">
      <div class="mr-4">範囲指定</div>
      <input
        v-model="rangeBegin"
        class="w-20 rounded-md border-2 border-solid border-violet-300 p-1 px-3 text-right text-lg"
        type="number"
        placeholder="開始ページ"
      />
      <span class="me-2 ms-2">-</span>
      <input
        v-model="rangeEnd"
        class="w-20 rounded-md border-2 border-solid border-violet-300 p-1 px-3 text-right text-lg"
        type="number"
        placeholder="終了ページ"
      />
    </div>

    <div class="flex items-center">
      <div class="mr-4">出力先</div>
      <button
        class="flex min-w-10 items-center gap-1 rounded-md bg-neutral-300 p-1 px-2 leading-none"
        @click="openDialog"
      >
        <span class="i-bx-folder-plus size-5 bg-violet-500"></span>
        <span
          v-if="outDir.length !== 0"
          v-text="outDir"
        ></span>
        <span v-else>フォルダを選択</span>
      </button>
    </div>
  </div>
  <span class="m-8 block h-0 border-[1px] border-solid border-violet-200"></span>
</template>

<style scoped></style>
