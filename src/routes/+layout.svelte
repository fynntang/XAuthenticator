<script lang="ts">
    import '../app.css';
    import {ModeWatcher} from "mode-watcher";
    import type {LayoutProps} from "./$types";


    let {children}: LayoutProps = $props();
</script>

<ModeWatcher/>

{@render children()}
