<script lang="ts">
    import '../app.css';
    import {ModeWatcher} from "mode-watcher";
    import type {LayoutProps} from "./$types";
    import '$lib/i18n';
    import {Toaster} from "$lib/components/ui/sonner";


    let {children}: LayoutProps = $props();
</script>

<ModeWatcher/>
<Toaster/>

{@render children()}
