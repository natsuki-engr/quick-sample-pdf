export interface FileInfo {
  exists: boolean
  path: string
  page_num: number
}

export const isFileInfo = (data: unknown): data is FileInfo => {
  return (
    typeof data === 'object' &&
    data !== null &&
    'exists' in data &&
    typeof data.exists === 'boolean' &&
    'path' in data &&
    typeof data.path === 'string' &&
    'page_num' in data &&
    typeof data.page_num === 'number'
  )
}
