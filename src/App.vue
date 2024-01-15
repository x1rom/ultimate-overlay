<template>
    <v-app class="bg-grey-darken-4">
        <v-container class="px-12 py-6" style="height: 350px">
            <v-form @submit.prevent>
                <v-row>
                    <v-col class="h-25">
                        <p class="mb-5 font-weight-bold float-left text-h5">
                            Player 1
                        </p>
                        <v-row>
                            <v-text-field
                                label="Score"
                                dirty
                                class="pa-1 w-0 font-weight-bold"
                                type="number"
                                no-resize
                                rows="1"
                                variant="outlined"
                                rounded
                                v-model="score_l"
                            />
                            <v-text-field
                                label="Name"
                                dirty
                                class="pa-1 w-auto font-weight-bold"
                                no-resize
                                rows="1"
                                variant="outlined"
                                v-model="name_l"
                                clearable
                            />
                        </v-row>
                    </v-col>
                    <v-col class="h-25">
                        <p class="mb-5 font-weight-bold float-right text-h5">
                            Player 2
                        </p>
                        <v-row>
                            <v-text-field
                                label="Name"
                                dirty
                                class="pa-1 w-auto font-weight-bold"
                                no-resize
                                rows="1"
                                variant="outlined"
                                v-model="name_r"
                                clearable
                            />
                            <v-text-field
                                label="Score"
                                dirty
                                class="pa-1 w-0 font-weight-bold"
                                type="number"
                                no-resize
                                rows="1"
                                variant="outlined"
                                rounded
                                v-model="score_r"
                            />
                        </v-row>
                    </v-col>
                </v-row>
                <v-divider class="mt-5 mb-8" />
                <v-row>
                    <v-col class="p-0 m-0">
                        <v-text-field
                            label="Tournament name"
                            dirty
                            class="w-100 font-weight-bold"
                            no-resize
                            rows="1"
                            variant="outlined"
                            v-model="tournament_name"
                            clearable
                        />
                    </v-col>
                    <v-col class="p-0 m-0">
                        <v-text-field
                            label="Round name"
                            dirty
                            class="w-100 font-weight-bold"
                            no-resize
                            rows="1"
                            variant="outlined"
                            v-model="round_name"
                            clearable
                        />
                    </v-col>
                </v-row>
                <v-divider class="mt-2 mb-10" />
                <v-row>
                    <v-col class="p-0 m-0">
                        <v-text-field
                            label="Caster 1"
                            dirty
                            class="w-100 font-weight-bold"
                            no-resize
                            rows="1"
                            variant="outlined"
                            v-model="caster1"
                            clearable
                        />
                    </v-col>
                    <v-col class="p-0 m-0">
                        <v-text-field
                            label="Caster2"
                            dirty
                            class="w-100 font-weight-bold"
                            no-resize
                            rows="1"
                            variant="outlined"
                            v-model="caster2"
                            clearable
                        />
                    </v-col>
                </v-row>
                <v-divider class="mt-2 mb-10" />
                <v-row justify="center">
                    <v-skeleton-loader
                        v-if="loading"
                        type="image"
                        class="flex-fill"
                        color="grey-darken-1"
                        style="
                            overflow: hidden;
                            height: 36px;
                            border-radius: 30px;
                        "
                    />
                    <v-btn
                        v-else
                        id="update-button"
                        class="bg-green-darken-4 flex-fill"
                        text="update"
                        rounded
                        type="submit"
                        @click="() => updateData()"
                    />
                </v-row>
            </v-form>
        </v-container>
    </v-app>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { Ref, ref } from "vue";
import OverlayData from "./OverlayData";

const score_l: Ref<string | null> = ref("0");
const score_r: Ref<string | null> = ref("0");
const name_l: Ref<string | null> = ref("");
const name_r: Ref<string | null> = ref("");
const tournament_name: Ref<string | null> = ref("");
const round_name: Ref<string | null> = ref("");
const caster1: Ref<string | null> = ref("");
const caster2: Ref<string | null> = ref("");

const loading: Ref<boolean> = ref(false);

function updateData() {
    loading.value = true;
    const data: OverlayData = {
        name_l: name_l.value ? name_l.value : "",
        name_r: name_r.value ? name_r.value : "",
        score_l: score_l.value ? +score_l.value : 0,
        score_r: score_r.value ? +score_r.value : 0,
        tournament_name: tournament_name.value ? tournament_name.value : "",
        round_name: round_name.value ? round_name.value : "",
        casters: [
            caster1.value ? caster1.value : "",
            caster2.value ? caster2.value : "",
        ],
    };
    invoke("set_data", { data: data });
    setTimeout(() => {
        loading.value = false;
    }, 500);
}
</script>
