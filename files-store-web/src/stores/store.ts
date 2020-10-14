import { writable } from "svelte/store"

import type { FsNode } from "../FsNode"

export const wantCreateDirectory = writable(false)

function createFsNodesStore() {
  const { subscribe, set, update } = writable<FsNode[]>([])

  return {
    subscribe,
    set: (fsNodes: FsNode[]) => {
      set(fsNodes)
      return fsNodes
    },
    addDirectory: (fsNode: FsNode) => update(files => [fsNode, ...files]),
    add: (fsNode: FsNode[]) => update(files => files.concat(fsNode))
  }
}

export const fsNodesStore = createFsNodesStore()
