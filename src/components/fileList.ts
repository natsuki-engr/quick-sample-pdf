export interface EmitType {
  (e: 'remove-file', value: number): void
}

export const setup = (emits: EmitType) => {
  const removeFile = (i: number) => {
    emits('remove-file', i)
  }

  return {
    removeFile,
  }
}
