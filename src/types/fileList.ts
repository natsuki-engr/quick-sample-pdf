export interface FileInfo {
  exists: boolean
  dir: string
  pageNum: number
  fileName: string
  // saved: boolean
  status: FileStatusType
  pageRange: [number, number] | null
}

export const FileStatus = {
  LOADED: 'loaded',
  SAVED: 'saved',
  SAVE_FAILED: 'save-failed'
} as const

export type FileStatusType = typeof FileStatus[keyof typeof FileStatus]

export const isFileInfo = (data: unknown): data is FileInfo => {
  return (
    typeof data === 'object' &&
    data !== null &&
    'exists' in data &&
    typeof data.exists === 'boolean' &&
    'dir' in data &&
    typeof data.dir === 'string' &&
    'pageNum' in data &&
    typeof data.pageNum === 'number' &&
    'fileName' in data &&
    typeof data.fileName === 'string'
  )
}

export interface FileInfoResponse {
  exists: boolean
  path: string
  page_num: number
}

export const isFileInfoResponse = (data: unknown): data is FileInfoResponse => {
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
