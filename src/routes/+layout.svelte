<script lang="ts">
    import '../app.css';
    import {ModeWatcher} from "mode-watcher";
    import {onMount} from "svelte";

    let {children} = $props();

    onMount(()=>{

        // init_app


    })

</script>

<ModeWatcher/>
{@render children()}
