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
  <div class="rounded-lg border-violet-500 bg-white p-6">
    <div class="flex items-center">
      <div class=" mr-4">範囲指定</div>
      <input
        v-model="rangeBegin"
        class="w-28 rounded-md border-2 border-solid border-violet-300 p-1 text-lg"
        type="number"
        placeholder="開始ページ"
      />
      <span class=" me-2 ms-2">-</span>
      <input
        v-model="rangeEnd"
        class="w-28 rounded-md border-2 border-solid border-violet-300 p-1 text-lg"
        type="number"
        placeholder="終了ページ"
      />
    </div>

    <div>
      <span>出力先</span>
      <button
        @click="openDialog"
        v-text="outDir"
      ></button>
    </div>
  </div>
  <span class="m-8 block h-0 border-[1px] border-solid border-violet-200"></span>
</template>

<style scoped></style>
