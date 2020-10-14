<script lang="typescript">
  export let name: string
  export let parentUuid: string
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"
  import { createDirectory } from "../FileStoreApi"
  import { fsNodesStore, wantCreateDirectory } from "../stores/store"

  function handleBlur() {
    if (name !== "") {
      createDirectory(parentUuid, name)
        .then(dir => {
          fsNodesStore.add([dir])
          wantCreateDirectory.update(() => false)
        })
        .catch(console.error)
    } else {
      wantCreateDirectory.update(() => false)
    }
  }
</script>

<div class="fs-node">
  <div class="icon">
    <DirectoryIcon />
  </div>
  <input on:blur="{handleBlur}" type="text" bind:value="{name}" />
</div>

<style>
  .fs-node {
    display: flex;
    align-items: center;
    padding: 15px;
    border-bottom: 1px solid var(--border);
  }

  .icon {
    width: 30px;
    margin-right: 15px;
    margin-top: 2px;
  }

  input {
    color: var(--primary-text);
  }
</style>
