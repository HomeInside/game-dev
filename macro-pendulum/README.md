## Péndulos

El [péndulo](https://es.wikipedia.org/wiki/P%C3%A9ndulo) es un sistema físico que puede oscilar bajo la acción gravitatoria u otra característica física (elasticidad, por ejemplo) y que está configurado por una masa suspendida de un punto o de un eje horizontal fijos mediante un hilo, una varilla, u otro dispositivo que pueda mantener fijo el sistema.

Para este ejemplo se crea un [péndulo simple (también llamado péndulo matemático o péndulo ideal)](https://es.wikipedia.org/wiki/P%C3%A9ndulo_simple) es un sistema idealizado constituido por una partícula de masa m que está suspendida de un punto fijo o mediante un hilo al que se le puede regular su longitud y su peso.

El péndulo simple o matemático se denomina así en contraposición a los péndulos reales, compuestos o físicos, únicos que pueden construirse.

Podemos describir el péndulo con:

* `L` → longitud de la cuerda/varilla.
* `m` → masa de la bola.
* `g` → gravedad, aproximadamente `9.81 m/s²`.
* `tetha: θ` → ángulo respecto de la vertical.
* `omega: ω = dθ/dt` = velocidad angular.
* `alpha: α = d²θ/dt²` → aceleración angular.
* `damping` = amortiguamiento.
* `pivot_x/pivot_y` = posición del pivote.
* `bob_x/bob_y` = posición de la bola.

Visualmente:

```
    ●  ← pivote
    |\
    | \
    |  \ L
    | θ \
    |    \
    |     ●  ← masa
    |
    ↓
    gravedad
```

como **no necesitamos calcular la posición de la bola directamente**, primero calculamos cómo cambia `θ`(el ángulo respecto de la vertical), y a partir de esta, obtenemos la posición.

como la gravedad apunta siempre hacia abajo...
```
    ●
    |
    |\
    | \
    |  ●
    |
    ↓ g
```

nos interesa la parte de la gravedad que realmente hace girar el péndulo, el componente tangencial del peso:

```math
F_t = -mg\sin(\theta)
```

El signo **`-`** es importante: significa que la fuerza intenta llevar el péndulo hacia `θ = 0`(punto de equilibrio), en otras palabras, la fuerza tangencial tiene siempre sentido opuesto al desplazamiento (fuerza recuperadora).


Por eso, cuando la bola está a la derecha:

```text
       ●
        \
         \
          ●
         ←
```

la gravedad produce una aceleración hacia la izquierda y viceversa.


## calculo de la posición

Para el ejemplo, el pivote está arriba, y si `θ != 0`, se crea un ángulo respecto a la vertical, así que tenemos un triángulo rectángulo:

```
    ● <- Pivote
    |\
    | \
  y |  \ L
    |   \
    |    \
    | θ   ● <- bola
    +------
    x
```

Tenemos:

* `L` → hipotenusa (longitud de la cuerda)
* `x` → desplazamiento horizontal (cateto opuesto)
* `y` → desplazamiento vertical (cateto adyacente)
* `θ` → ángulo

Y aquí aparecen seno y coseno.

### Seno

Por definición:

```math
\sin(\theta)
\frac{\text{cateto opuesto}}{\text{hipotenusa}}
```

El cateto opuesto a `θ` es `x`.

Entonces:

```math
\sin(\theta) = \frac{x}{L}
```

Multiplicamos por `L`:

```math
\boxed{x = L\sin(\theta)}
```

Es decir: ¿Cuánto se ha desplazado horizontalmente la bola?


### Coseno

Ahora hacemos exactamente lo mismo.

Por definición:

```math
\cos(\theta)
\frac{\text{cateto adyacente}}{\text{hipotenusa}}
```

El cateto adyacente es la distancia vertical:

```math
\cos(\theta) = \frac{y}{L}
```

Por tanto:

```math
\boxed{y = L\cos(\theta)}
```


## referencias

- [El péndulo simple](http://www.sc.ehu.es/sbweb/fisica/dinamica/trabajo/pendulo/pendulo.htm)
- [péndulo](https://www.studysmarter.es/resumenes/fisica/oscilaciones/pendulo/)
- [Trigonometría y fuerzas: el péndulo](https://es.khanacademy.org/computing/computer-programming/programming-natural-simulations/programming-oscillations/a/trig-and-forces-the-pendulum)
- [Fisica2-Unidad2-El Pendulo Fisico](https://www.youtube.com/watch?v=SjkJS7P98d4)
- [Teoría del péndulo simple.](https://www.youtube.com/watch?v=YfJB-PakY-U)
- [Ley de elasticidad de Hooke](https://es.wikipedia.org/wiki/Ley_de_elasticidad_de_Hooke)
- [Velocidad angular](https://es.wikipedia.org/wiki/Velocidad_angular)
- [Aceleración angular](https://es.wikipedia.org/wiki/Aceleraci%C3%B3n_angular)
- [Segunda ley de Newton](https://es.wikipedia.org/wiki/Leyes_de_Newton#Segunda_ley_de_Newton_o_ley_fundamental_de_la_din%C3%A1mica)
- [aceleraciones tangencial y normal](https://es.wikipedia.org/wiki/Aceleraci%C3%B3n#Componentes_intr%C3%ADnsecas_de_la_aceleraci%C3%B3n:_aceleraciones_tangencial_y_normal)
- [Péndulo simple](https://es.wikipedia.org/wiki/P%C3%A9ndulo_simple)
-
