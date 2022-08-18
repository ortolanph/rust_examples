param ($project_name)

Write-Output "Creating project $project_name"

cargo new $project_name --bin

cd $project_name

Write-Output "Project $project_name created with success."
pwd