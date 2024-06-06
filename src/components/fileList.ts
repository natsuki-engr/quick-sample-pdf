export interface EmitType {
  (e: 'click-start-btn'): void
  (e: 'remove-file', value: number): void
}

export const setup = (emits: EmitType) => {
  const onClickStartBtn = () => {
    console.log('onClickStartBtn')
    emits('click-start-btn')
  }

  const removeFile = (i: number) => {
    emits('remove-file', i)
  }

  return {
    onClickStartBtn,
    removeFile,
  }
}
