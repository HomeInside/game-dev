# macro-gamepad

Este ejemplo utiliza [GilRs](https://gitlab.com/gilrs-project/gilrs) que abstrae las API específicas de cada plataforma para proporcionar interfaces unificadas con las que trabajar con mandos de videojuegos.

Características principales:
- Disposición unificada del mando: los botones y los ejes se representan con nombres conocidos
- Compatibilidad con las asignaciones de `SDL2`, incluida la variable de entorno `SDL_GAMECONTROLLERCONFIG` que utiliza [Steam](https://store.steampowered.com/)
- Conexión en caliente: *GilRs* intentará asignar nuevos ID a los mandos nuevos y reutilizará el mismo ID para que los mandos que se vuelvan a conectar
- Retroalimentación de fuerza ("Force Feedback", la vibración)
- Información sobre la alimentación (si el mando es con cable, estado actual de la batería).

## WGI y XInput

**Windows** utiliza por defecto **Windows Gaming Input** en lugar de **XInput**. Si necesitas utilizar XInput, puedes desactivar la función wgi (está activada por defecto) y activar la función xinput.
*Windows Gaming Input* requiere que haya una ventana activa asociada al proceso para recibir eventos. Aún así, puedes volver a utilizar XInput desactivando las funciones predeterminadas y activando la función XInput.

> Nota:
    Es posible que algunos dispositivos (¿más antiguos?) sigan notificando entradas sin una ventana activa, pero este no es el caso
    para todos los dispositivos, por lo que, si estás desarrollando un juego basado en terminal, utiliza la función XInput en su lugar.

## posible doble reporte de teclas

como se mencionó arriba, *Windows*, expone los gamepads a través de dos backends: *WGI* y *XInput*, El DualShock 4 (usado en este ejemplo), al ser un dispositivo compatible con ambos, reporta sus eventos a los dos sistemas. *GilRs*, por defecto, escucha ambos flujos, lo que puede resultar en el doble log(se reporta dos veces la misma tecla ó stick)

## deadzone(drift)

Una zona muerta(deadzone) es un radio de umbral alrededor del centro de un joystick analógico en el que cualquier movimiento se considera una entrada nula. Su función es suprimir el ruido del hardware: pequeños valores distintos de cero que el joystick registra incluso cuando no se toca, debido a la imprecisión del sensor o al desgaste mecánico.

al no tocarlo:
```
X = 0
Y = 0
```

pero en la realidad puede devolver algo como:

```
X = 0.03
Y = -0.02
```
aunque tú no estés moviendo el stick, así que el juego interpreta esos pequeños valores como movimiento. Por eso un personaje puede caminar lentamente hacia un lado, una cámara puede girar sola, etc.


## referencias

- [GilRs](https://gitlab.com/gilrs-project/gilrs)
- [asciiflow](https://asciiflow.com/#/)
- [ds4-windows](https://ds4-windows.com/)
- [gamepadtester](https://gamepadtester.co.uk/deadzone-test/)
