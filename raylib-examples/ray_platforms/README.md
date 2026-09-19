# ray_platforms

## Plataformas fijas

Son superficies que no se mueven, se colocan en una posición del escenario y ahí se quedan, son el tipo más básico de plataforma y suelen servir como:

- suelo, paredes ó techos.
- escalones(escaleras).
- bloques decorativos o de ambientación.
- bloques fijos para colisiones
- la base sobre la que se apoyan otras mecánicas

## Plataformas móviles

Son obstáculos que se desplazan solos por el escenario, rebotando entre dos puntos, a una velocidad constante(normalmente), y cambian de dirección automáticamente al llegar al límite de su recorrido y suelen servir como:

- ascensores
- obstáculos que se cruzan
- bloques móviles para colisiones
- la base sobre la que se apoyan otras mecánicas

```
┌────────────────────────────────────────────────────┐
│          m                           m             │
│     ┌────────┐                     ┌─────┐         │
│     └────────┘                     └─────┘         │
│                                                    │
│                                                    │
│                                                    │
│                          f                         │
│                         ┌────────┐                 │
│                    f    └────────┘                 │
│           f      ┌────────┐                        │
│      ┌─────────┐ └────────┘                        │
│      └─────────┘                                   │
└────────────────────────────────────────────────────┘
```

## referencias

- [raylib cheatsheet](https://www.raylib.com/cheatsheet/cheatsheet.html)
- [asciiflow](https://asciiflow.com/#/)
