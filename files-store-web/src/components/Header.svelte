<script lang="typescript">
  import { _ } from "svelte-intl"
</script>

<header>
  <a class="home" href="/">{$_('title')}</a>
  <nav><a class="link" href="#/bin">{$_('bin')}</a></nav>
</header>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 60px;
    padding-left: 15px;
    padding-right: 15px;
    background-color: var(--primary);
  }

  .home {
    font-size: 1.1rem;
    color: white;
    text-decoration: none;
  }
  .home:hover {
    text-decoration: underline;
  }

  .link {
    color: white;
    text-decoration: none;
  }
  .link:hover {
    text-decoration: underline;
  }
</style>
