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

### posible doble reporte de teclas

como se mencionó arriba, *Windows*, expone los gamepads a través de dos backends: *WGI* y *XInput*, El DualShock 4 (usado en este ejemplo), al ser un dispositivo compatible con ambos, reporta sus eventos a los dos sistemas. *GilRs*, por defecto, escucha ambos flujos, lo que puede resultar en el doble log(se reporta dos veces la misma tecla ó stick)


## referencias

- [GilRs](https://gitlab.com/gilrs-project/gilrs)
- [asciiflow](https://asciiflow.com/#/)
- [ds4-windows](https://ds4-windows.com/)
