<script lang="typescript">
  import { fly } from "svelte/transition"
  import { quintOut } from "svelte/easing"
</script>

<div class="flaying-panel" transition:fly="{{ duration: 200, y: 200, easing: quintOut }}">
  <slot />
</div>

<style>
  .flaying-panel {
    padding: 20px;
    background-color: var(--secondary);
    border-top: 1px solid var(--primary);
    border-top-left-radius: 15px;
    border-top-right-radius: 15px;
    box-shadow: 0px 0px 3px 0px rgba(0, 0, 0, 0.2);
    height: 40vh;
  }
</style>
