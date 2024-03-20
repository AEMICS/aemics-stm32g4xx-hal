#[allow(clippy::upper_case_acronyms)]
pub enum DmaMuxResources {
    DMAMUXReqG0 = 1,
    DMAMUXReqG1 = 2,
    DMAMUXReqG2 = 3,
    DMAMUXReqG3 = 4,
    ADC1 = 5,
    DAC1_CH1 = 6,
    DAC1_CH2 = 7,
    TIM6_UP = 8,
    TIM7_UP = 9,
    SPI1_RX = 10,
    SPI1_TX = 11,
    SPI2_RX = 12,
    SPI2_TX = 13,
    SPI3_RX = 14,
    SPI3_TX = 15,
    I2C1_RX = 16,
    I2C1_TX = 17,
    I2C2_RX = 18,
    I2C2_TX = 19,
    I2C3_RX = 20,
    I2C3_TX = 21,
    I2C4_RX = 22,
    I2C4_TX = 23,
    USART1_RX = 24,
    USART1_TX = 25,
    USART2_RX = 26,
    USART2_TX = 27,
    USART3_RX = 28,
    USART3_TX = 29,
    USART4_RX = 30,
    USART4_TX = 31,
    USART5_RX = 32,
    USART5_TX = 33,
    LPUART1_RX = 34,
    LPUART1_TX = 35,
    ADC2 = 36,
    ADC3 = 37,
    ADC4 = 38,
    ADC5 = 39,
    QUADSPI = 40,
    DAC2_CH1 = 41,
    TIM1_CH1 = 42,
    TIM1_CH2 = 43,
    TIM1_CH3 = 44,
    TIM1_CH4 = 45,
    TIM1_UP = 46,
    TIM1_TRIG = 47,
    TIM1_COM = 48,
    TIM8_CH1 = 49,
    TIM8_CH2 = 50,
    TIM8_CH3 = 51,
    TIM8_CH4 = 52,
    TIM8_UP = 53,
    TIM8_TRIG = 54,
    TIM8_COM = 55,
    TIM2_CH1 = 56,
    TIM2_CH2 = 57,
    TIM2_CH3 = 58,
    TIM2_CH4 = 59,
    TIM2_UP = 60,
    TIM3_CH1 = 61,
    TIM3_CH2 = 62,
    TIM3_CH3 = 63,
    TIM3_CH4 = 64,
    TIM3_UP = 65,
    TIM3_TRIG = 66,
    TIM4_CH1 = 67,
    TIM4_CH2 = 68,
    TIM4_CH3 = 69,
    TIM4_CH4 = 70,
    TIM4_UP = 71,
    TIM5_CH1 = 72,
    TIM5_CH2 = 73,
    TIM5_CH3 = 74,
    TIM5_CH4 = 75,
    TIM5_UP = 76,
    TIM15_CH1 = 77,
    TIM15_CH2 = 78,
    TIM15_CH3 = 79,
    TIM15_CH4 = 80,
    TIM15_UP = 81,
    TIM16_CH1 = 82,
    TIM16_UP = 83,
    TIM17_CH1 = 84,
    TIM17_UP = 85,
    TIM20_CH1 = 86,
    TIM20_CH2 = 87,
    TIM20_CH3 = 88,
    TIM20_CH4 = 89,
    TIM20_UP = 90,
    AES_IN = 91,
    AES_OUT = 92,
    TIM20_TRIG = 93,
    TIM20_COM = 94,
    HRTIM_MASTER = 95,
    HRTIM_TIMA = 96,
    HRTIM_TIMB = 97,
    HRTIM_TIMC = 98,
    HRTIM_TIMD = 99,
    HRTIM_TIME = 100,
    HRTIM_TIMF = 101,
    DAC3_CH1 = 102,
    DAC3_CH2 = 103,
    DAC4_CH1 = 104,
    DAC4_CH2 = 105,
    SPI4_RX = 106,
    SPI4_TX = 107,
    SAI1_A = 108,
    SAI1_B = 109,
    FMAC_Read = 110,
    FMAC_Write = 111,
    Cordic_Read = 112,
    Cordic_Write = 113,
    UCPD1_RX = 114,
    UCPD2_TX = 115,
    // Reserved_116 = 116,
    // Reserved_117 = 117,
    // Reserved_118 = 118,
    // Reserved_119 = 119,
    // Reserved_120 = 120,
    // Reserved_121 = 121,
    // Reserved_122 = 122,
    // Reserved_123 = 123,
    // Reserved_124 = 124,
    // Reserved_125 = 125,
    // Reserved_126 = 126,
    // Reserved_127 = 127,
}

impl From<DmaMuxResources> for u8 {
    fn from(dmr: DmaMuxResources) -> u8 {
        dmr as u8
    }
}
