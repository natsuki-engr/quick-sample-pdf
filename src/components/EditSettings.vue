<script setup lang="ts">
import { setup } from './editSettings';

const {
  rangeBegin,
  rangeEnd,
} = setup()

</script>

<template>
  <div>
    <span>ファイル分割</span>
    <input v-model="rangeBegin" type="number">
    <span>~</span>
    <input v-model="rangeEnd" type="number">
  </div>
</template>

<style scoped></style>
