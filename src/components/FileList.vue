<script setup lang="ts">
import { type PropType } from 'vue'
import { FileInfo } from '../types/fileList'
import { setup } from './fileList'
import { EmitType } from './fileList'
import FileCard from './FileCard.vue'

const emits = defineEmits<EmitType>()

const { removeFile } = setup(emits)

defineProps({
  loading: {
    type: Boolean,
    required: true,
  },
  fileList: {
    type: Array as PropType<Array<FileInfo>>,
    required: false,
    default: () => [],
  },
})
</script>

<template>
  <div>
    <TransitionGroup
      enter-active-class="duration-300 ease-out group-transition-delay"
      enter-from-class="opacity-0 translate-y-32"
      enter-to-class="opacity-1"
      leave-active-class="duration-500"
      leave-from-class="opacity-1"
      leave-to-class="opacity-0 translate-x-20"
    >
      <FileCard
        v-for="(file, i) of fileList"
        :key="file.dir + file.fileName"
        :file="file"
        :style="{ '--i': `${i + 1}00ms` }"
        @click-remove-btn="removeFile(i)"
      ></FileCard>
    </TransitionGroup>
  </div>
</template>
