package com.aicode.intellij.settings;

import com.intellij.openapi.components.*;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@State(
    name = "AiCodeSettings",
    storages = {@Storage("aicode.xml")}
)
@Service
public class AiCodeSettings implements PersistentStateComponent<AiCodeSettings.State> {
    private State myState = new State();

    public static class State {
        public String apiKey = "";
        public boolean enableAutoComplete = true;
    }

    public static AiCodeSettings getInstance(Project project) {
        return project.getService(AiCodeSettings.class);
    }

    @Nullable
    @Override
    public State getState() {
        return myState;
    }

    @Override
    public void loadState(@NotNull State state) {
        myState = state;
    }
}
