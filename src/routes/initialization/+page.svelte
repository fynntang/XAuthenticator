<script lang="ts">
    import type {PageProps} from './$types';
    import {randomLaunchImage} from "$lib/utils";

    let {params, data}: PageProps = $props();

    $inspect(params, data)
</script>


<main data-tauri-drag-region class="relative flex min-h-screen w-screen items-center justify-center p-4"
      style:background="url({randomLaunchImage()}) center/cover no-repeat">

</main>

<style lang="scss">
  main {
    &::before {
      position: absolute;
      content: "";
      background: inherit;
      filter: blur(10px);
      transform: scale(1.1);
      z-index: 0;
    }

    &::after {
      position: absolute;
      content: "";
      inset: 0;
      background-color: color-mix(in oklab, var(--color-black) 10%, transparent);
      backdrop-filter: blur(10px);
      z-index: 1;
    }
  }
</style>
