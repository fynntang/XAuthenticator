<script lang="ts">
    import {appSettingsStore} from "$lib/stores/app-settings";
    import {Dialog} from "$lib/components/ui/dialog";
    import {onMount} from "svelte";

    onMount(() => {
        console.log("appSettingsStore:", $appSettingsStore);
    })
</script>


<Dialog open={$appSettingsStore}>

    <h1>Settings</h1>
</Dialog>
