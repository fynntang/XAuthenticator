<script lang="ts">
    import type {PageProps} from './$types';

    import {Button} from "$lib/components/ui/button";
    import {Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle} from "$lib/components/ui/card";
    import {InputGroup, InputGroupAddon, InputGroupInput} from "$lib/components/ui/input-group";
    import {Alert, AlertDescription, AlertTitle} from "$lib/components/ui/alert";
    import {Progress} from "$lib/components/ui/progress";
    import {AlertCircle, Eye, EyeOff, KeyRound, Lock} from "@lucide/svelte";
    import {Spinner} from "$lib/components/ui/spinner";

    let {params, data}: PageProps = $props();
    let password = $state("");
    let confirmPassword = $state("");
    let showPassword = $state(false);
    let showConfirm = $state(false);
    let acknowledged = $state(false);
    let error = $state("");
    let loading = $state(false);

    type Strength = { score: number; label: string; color: string; hints: string[] };
    let strength: Strength = $state({score: 0, label: "弱", color: "red", hints: []});

    const hasUpper = (s: string) => /[A-Z]/.test(s);
    const hasLower = (s: string) => /[a-z]/.test(s);
    const hasDigit = (s: string) => /\d/.test(s);
    const hasSymbol = (s: string) => /[^A-Za-z0-9]/.test(s);

    const evaluateStrength = (pw: string): Strength => {
        const hints: string[] = [];
        let score = 0;
        if (pw.length >= 12) score += 1; else hints.push("至少 12 个字符");
        if (hasUpper(pw) && hasLower(pw)) score += 1; else hints.push("包含大小写字母");
        if (hasDigit(pw)) score += 1; else hints.push("包含数字");
        if (hasSymbol(pw)) score += 1; else hints.push("包含符号（如 !@#）");
        const labels = ["弱", "一般", "中等", "良好", "强"];
        const colors = ["red", "orange", "yellow", "emerald", "green"];
        const idx = Math.min(score, 4);
        return {score, label: labels[idx], color: colors[idx], hints};
    };

    $effect(() => {
        strength = evaluateStrength(password);
        error = ""; // 清空错误以便重新校验
    });

    const validate = (): boolean => {
        if (!password || !confirmPassword) {
            error = "请输入主密码并再次确认";
            return false;
        }
        if (password !== confirmPassword) {
            error = "两次输入的密码不一致";
            return false;
        }
        if (strength.score < 3) {
            error = "密码强度过低，请提高复杂度";
            return false;
        }
        if (!acknowledged) {
            error = "请确认已妥善保存主密码";
            return false;
        }
        return true;
    };

    const onInitialize = async () => {
        error = "";
        if (!validate()) return;
        loading = true;
        openConfirm = true;

        console.log("开始初始化");
    };


    $inspect(params, data, loading)
</script>


<main data-tauri-drag-region class="flex min-h-screen w-screen items-center justify-center p-4">
    <Card data-tauri-drag-region class="w-full max-w-md">
        <CardHeader>
            <CardTitle class="flex items-center gap-2">
                <Lock class="text-muted-foreground"/>
                需要初始化MasterKey
            </CardTitle>
            <CardDescription>
                为保障本地数据安全，请先设置主密码以完成 MasterKey 初始化。
            </CardDescription>
        </CardHeader>
        <CardContent class="grid gap-4">
            <div class="flex items-start gap-3 text-sm text-muted-foreground">
                <KeyRound class="mt-0.5 size-4"/>
                <div>
                    <div class="font-medium text-foreground">初始化步骤</div>
                    <ul class="mt-1 list-disc pl-5">
                        <li>设置并确认主密码</li>
                        <li>二次确认以防误操作</li>
                        <li>完成后自动继续应用初始化</li>
                    </ul>
                </div>
            </div>
            <div class="grid gap-3">
                <label class="text-sm font-medium">主密码</label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showPassword ? "text" : "password"} bind:value={password}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="请输入主密码（建议12位以上）"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showPassword = !showPassword)}
                                aria-label={showPassword ? "隐藏密码" : "显示密码"}>
                            {#if showPassword}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>

                <div class="flex items-center gap-2">
                    <span class="text-xs text-muted-foreground">强度：</span>
                    <span class="text-xs font-medium" class:!text-red-600={strength.color === 'red'}
                          class:!text-orange-600={strength.color === 'orange'}
                          class:!text-yellow-600={strength.color === 'yellow'}
                          class:!text-emerald-600={strength.color === 'emerald'}
                          class:!text-green-600={strength.color === 'green'}>{strength.label}</span>
                </div>
                <Progress max={4} value={strength.score}/>
                {#if strength.hints.length}
                    <div class="text-xs text-muted-foreground">建议：{strength.hints.join("，")}</div>
                {/if}
            </div>

            <div class="grid gap-3">
                <label class="text-sm font-medium">确认主密码</label>
                <InputGroup aria-invalid={!!error}>
                    <InputGroupInput type={showConfirm ? "text" : "password"} bind:value={confirmPassword}
                                     autocomplete="new-password" autocapitalize="off" spellcheck={false}
                                     placeholder="请再次输入主密码"/>
                    <InputGroupAddon align="inline-end">
                        <Button variant="ghost" size="icon" onclick={() => (showConfirm = !showConfirm)}
                                aria-label={showConfirm ? "隐藏密码" : "显示密码"}>
                            {#if showConfirm}
                                <EyeOff/>
                            {:else}
                                <Eye/>
                            {/if}
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
            </div>

            <label class="flex items-center gap-2 text-sm">
                <input type="checkbox" bind:checked={acknowledged} class="size-4 rounded border"/>
                我已妥善保存主密码，遗忘将无法找回。
            </label>

            {#if error}
                <Alert variant="destructive">
                    <AlertCircle/>
                    <AlertTitle>操作失败</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                </Alert>
            {/if}
        </CardContent>
        <CardFooter class="flex gap-2">
            <Button class="flex-1" onclick={onInitialize} disabled={loading}>
                {#if loading}
                    <Spinner/>
                {/if}
                下一步
            </Button>
        </CardFooter>
    </Card>
</main>
