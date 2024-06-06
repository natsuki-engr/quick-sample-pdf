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
  <div>
    <span>ファイル分割</span>
    <input
      v-model="rangeBegin"
      class="w-10 rounded-md border-2 border-solid border-violet-300 p-1 text-lg"
      type="number"
    />
    <span>~</span>
    <input
      v-model="rangeEnd"
      class="w-10 rounded-md border-2 border-solid border-violet-300 p-1 text-lg"
      type="number"
    />
  </div>

  <div>
    <span>出力先</span>
    <button
      @click="openDialog"
      v-text="outDir"
    ></button>
  </div>
</template>

<style scoped></style>
