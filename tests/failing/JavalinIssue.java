package com.example;

import io.javalin.Javalin;
import io.javalin.apibuilder.EndpointGroup;
import io.javalin.http.Handler;
import io.javalin.http.HandlerType;
import io.javalin.http.staticfiles.ResourceHandler;
import io.javalin.security.RouteRole;
import io.javalin.websocket.WsConfig;
import io.javalin.websocket.WsHandlerType;
import org.jetbrains.annotations.NotNull;

import java.util.function.Consumer;

public class JavalinIssue {

    // This method caused Tree-sitter parsing issues in the Javalin codebase.
    // It involves annotations, varargs, and complex types in the method signature.
    @NotNull
    @Override
    public Javalin addWsHandler(@NotNull WsHandlerType handlerType, @NotNull String path, @NotNull Consumer<WsConfig> wsConfig, @NotNull RouteRole @NotNull ... roles) {
        // Simplified body to focus on the signature issue
        // Original: cfg.pvt.internalRouter.addWsHandler(handlerType, path, wsConfig, roles);
        return null; // Return null for simplicity in this test case
    }
}
