export type PropsType = {
  percentage: number;
  status: string;
}

export type EmitType = {
  (e: 'startGenerating'): void
}

export const setup = (emits: EmitType) => {
  const start = () => {
    emits('startGenerating')
  }

  return {
    start,
  }
}
