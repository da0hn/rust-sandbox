fn mover_zeros(nums: &mut [i32]) {
    let mut indice_nao_zero = 0;

    // Iterar através do array
    for i in 0..nums.len() {
        // Se o elemento atual for não zero
        if nums[i] != 0 {
            // Trocar o elemento não zero com o elemento no índice indice_nao_zero
            nums.swap(i, indice_nao_zero);
            // Avançar o índice indice_nao_zero
            indice_nao_zero += 1;
        }
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];

    // Chamar a função para mover os zeros para o final
    mover_zeros(&mut nums);

    // Imprimir o resultado
    println!("{:?}", nums);
}
