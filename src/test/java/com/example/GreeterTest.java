package com.example;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class GreeterTest {
    @Test
    public void testGreet() {
        Greeter greeter = new Greeter();
        assertEquals("Hello, World!", greeter.greet("World"));
    }
}
