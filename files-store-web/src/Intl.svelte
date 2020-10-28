<script context="module">
  import { locale, translations, getBrowserLocale } from "svelte-intl"

  translations.update({
    fr: {
      title: "Files Store",
      bin: "Corbeille",
      empty: "Vide",
      move: "Déplacer",
      delete: "Supprimer",
      createDirectory: "Créer un dossier",
      upload: "Upload"
    },
    en: {
      title: "Files Store",
      bin: "Bin",
      empty: "Empty",
      move: "Move",
      delete: "Delete",
      createDirectory: "Create Directory",
      upload: "Upload"
    }
  })

  locale.set(getBrowserLocale("fr"))
</script>
