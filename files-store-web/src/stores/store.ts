import { writable } from "svelte/store"

import type { FsNode } from "../FsNode"

export const wantCreateDirectory = writable(false)
export const wantMoveFsNode = writable<FsNode | undefined>(undefined)

function createSelectedFsNode() {
  const { subscribe, update } = writable<FsNode[]>([])
  return {
    subscribe,
    toggle: (fsNode: FsNode) => {
      update(nodes => {
        const finded = nodes.find(n => n.uuid === fsNode.uuid)
        if (finded) {
          return nodes.filter(n => n.uuid !== fsNode.uuid)
        } else {
          return [...nodes, fsNode]
        }
      })
    },
    close: () => update(() => [])
  }
}

export const selectedFsNode = createSelectedFsNode()

function createFsNodesStore() {
  const { subscribe, set, update } = writable<FsNode[]>([])

  return {
    subscribe,
    set: (fsNodes: FsNode[]) => {
      set(fsNodes)
      return fsNodes
    },
    move: (fsNode: FsNode, _: FsNode) => update(files => files.filter(f => f.uuid !== fsNode.uuid)),
    addDirectory: (fsNode: FsNode) => update(files => [fsNode, ...files]),
    add: (fsNode: FsNode[]) => update(files => files.concat(fsNode))
  }
}

export const fsNodesStore = createFsNodesStore()
