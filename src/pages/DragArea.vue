<script setup lang="ts">
import setup from "./dragArea"

const {
  openDialog,
  debug,
} = setup()

debug()

</script>

<template>
  <div class="max-w-96">
    <button @click="openDialog">+</button>
  </div>
</template>

<style scoped></style>
