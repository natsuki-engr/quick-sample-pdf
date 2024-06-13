<script setup lang="ts">
import { setup } from './startBtn'
import { PropsType, EmitType } from './startBtn'
import { ProcessStatus } from '../types/processStatus'

const props = defineProps<PropsType>()

const emits = defineEmits<EmitType>()

const { start } = setup(props, emits)
</script>

<template>
  <button
    v-if="status === ProcessStatus.FILE_LOADED"
    class="ms-auto w-48 rounded-xl border-4 border-violet-500 bg-violet-500 p-5 text-3xl text-violet-50 shadow-md"
    @click="start"
  >
    <span>複製を開始</span>
  </button>
  <button
    v-else-if="status === ProcessStatus.GENERATING"
    class="relative ms-auto w-48 overflow-hidden rounded-xl border-4 border-violet-500 bg-white p-5 text-3xl text-violet-50 shadow-md"
  >
    <span
      class="absolute bottom-0 left-0 top-0 block bg-violet-200 transition-all duration-1000"
      :style="{ right: `${100 - percentage}%` }"
    ></span>
    <span class="text-violet-400 drop-shadow-lg">複製中...</span>
  </button>
  <button
    v-else-if="status === ProcessStatus.DONE"
    class="relative ms-auto w-48 overflow-hidden rounded-xl border-4 border-violet-500 bg-violet-200 p-5 text-3xl text-violet-50 shadow-md"
  >
    <span class="text-violet-400 drop-shadow-lg">完了</span>
  </button>
</template>
