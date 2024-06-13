export type PropsType = {
  percentage: number;
  status: string;
}

export type EmitType = {
  (e: 'startGenerating'): void
}

export const setup = (props: PropsType, emits: EmitType) => {
  const start = () => {
    emits('startGenerating')
  }

  return {
    start,
  }
}
