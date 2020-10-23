<script lang="typescript">
  import { createEventDispatcher } from "svelte"
  import IconButton from "./IconButton.svelte"
  import CloseIcon from "../icons/CloseIcon.svelte"

  export let title: string

  const dispatch = createEventDispatcher()
  const close = () => dispatch("close")

  const handle_keydown = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      close()
    }
  }
</script>

<svelte:window on:keydown="{handle_keydown}" />

<div class="modal-background" on:click="{close}"></div>

<div class="modal" role="dialog" aria-modal="true">
  <div class="header">
    <h2 class="title">{title}</h2>
    <IconButton on:click="{close}">
      <CloseIcon size="{25}" />
    </IconButton>
  </div>
  <slot />
</div>

<style>
  .modal-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.3);
  }

  .modal {
    position: absolute;
    left: 50%;
    top: 50%;
    width: calc(100vw - 4em);
    max-width: 32em;
    min-height: 300px;
    max-height: calc(100vh - 4em);
    transform: translate(-50%, -50%);
    border-radius: 0.2em;
    background: white;
  }

  .header {
    padding: 15px;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
  }

  .title {
    font-size: 1rem;
    margin: 0;
    padding: 0;
    word-break: break-all;
    overflow: hidden;
  }
</style>
