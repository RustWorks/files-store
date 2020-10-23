<script lang="typescript">
</script>

<button on:click>
  <slot />
</button>

<style>
  button {
    appearance: none;
    border: none;
    background: none;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    transition: all 0.2s;
    cursor: pointer;
    border-radius: 3px;
    padding: 0;
    margin: 0;
  }

  button:hover {
    opacity: 0.8;
  }
</style>
