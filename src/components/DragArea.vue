<script setup lang="ts">
import { EmitType, setup } from './dragArea'

const emits = defineEmits<EmitType>()

const { openDialog } = setup(emits)
</script>

<template>
  <div class="flex h-full w-full items-center justify-center p-5">
    <div class="relative h-full max-h-[400px] w-full max-w-[1200px] rounded-md border-2 border-dashed border-violet-500">
      <button
        class="absolute inset-0 m-auto block h-16 w-16 rounded-lg border-2 border-solid border-violet-500 bg-white"
        @click="openDialog"
      >
        <span class="absolute inset-0 m-auto block h-4/5 w-1 rounded-full bg-violet-500"></span>
        <span class="absolute inset-0 m-auto block h-4/5 w-1 rotate-90 rounded-full bg-violet-500"></span>
      </button>
    </div>
  </div>
</template>

<style scoped></style>
