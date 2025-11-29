<script lang="ts">
    import AppLayout from "$lib/components/layout/app-layout.svelte";

    let {children} = $props();

</script>

<AppLayout>
    {@render children()}
</AppLayout>

<style lang="css">
</style>
