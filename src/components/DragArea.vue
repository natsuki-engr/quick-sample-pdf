<script setup lang="ts">
import { EmitType, setup } from "./dragArea";

const emits = defineEmits<EmitType>()

const {
  openDialog,
} = setup(emits)
</script>

<template>
  <div class="max-w-96">
    <button @click="openDialog">+</button>
  </div>
</template>

<style scoped></style>
