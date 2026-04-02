Goal
Create a shared HashMap that stores item names (String) and their quantities (i32).

Initialize the map with: {"Laptops": 10, "Phones": 20}.

Spawn 10 threads.

Each thread must try to "purchase" 1 Laptop and 1 Phone.

By the end, your inventory should exactly show {"Laptops": 0, "Phones": 10}.