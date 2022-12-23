![coil1](https://user-images.githubusercontent.com/90075803/209333543-8f65cd67-8284-4b57-a8ab-ec318283a0e7.png)
# coil
This offshoot of rot64 features a self-updating rule. The 64 bits are simultaneously the states being updated and the encoding of the update rule. The initial state-rule is shown numerically, but it's easy to decode whiteblock == 1 and blackblock == 0 to follow the computation. The nieghborhood is just the five bits centered on the cell to be updated.

![coil2](https://user-images.githubusercontent.com/90075803/209333568-feeb01af-e583-4c1b-939b-676abba9c459.png)
