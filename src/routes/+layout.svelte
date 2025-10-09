<script lang="ts">
    import '../app.css';
    import {ModeWatcher} from "mode-watcher";
    import AppSettings from '$lib/components/app-settings/app-settings.svelte';

    let {children} = $props();
</script>

<ModeWatcher/>
{@render children()}
